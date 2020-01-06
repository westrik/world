/*
TODO:
  - create a VPC + security group (just IPv4 for now)
  - set up EC2 instance
  - set up NLB with Elastic IP, point at EC2 box
  - security group should allow port http in and https out (for CodeDeploy)

  - set up RDS
  - set up IAM roles so things can talk properly
  - set up CodeDeploy to EC2 instances
  - set up S3
  - set up CloudFront
  - provision ACM private cert to use with NLB
  - set up new IAM role for scripted use (i.e. not root account)
*/

provider "aws" {
  region = "${var.aws_region}"
}

// TODO: IPv6
resource "aws_vpc" "ww_prod_app" {
  cidr_block = "10.0.0.0/16"

  tags = {
    Name = "ww_prod_app_vpc"
    Environment = "production"
  }
}

# security group for our app instances
resource "aws_security_group" "ww_prod_app" {
  name        = "ww_prod_app_sg"
  description = "Primary westrikworld production VPC"
  vpc_id      = "${aws_vpc.ww_prod_app.id}"

  # HTTP access from the VPC
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }

  # HTTPS access from the VPC
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }

  # SSH access from the VPC
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["10.0.0.0/16"]
  }

  # Outbound HTTPS access (for CodeDeploy)
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] # TODO: specify CIDR for CodeDeploy?
  }
}

resource "aws_internet_gateway" "ww_prod_app" {
  vpc_id = "${aws_vpc.ww_prod_app.id}"
}

resource "aws_route" "ww_prod_app_internet_access" {
  route_table_id         = "${aws_vpc.ww_prod_app.main_route_table_id}"
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = "${aws_internet_gateway.ww_prod_app.id}"
}

resource "aws_subnet" "ww_prod_app" {
  availability_zone = "us-east-1a"
  vpc_id                  = "${aws_vpc.ww_prod_app.id}"
  cidr_block              = "10.0.1.0/24"
//  map_public_ip_on_launch = true
  // (don't think we'll need public IPs since we're using an NLB)

  tags = {
    "Name" = "ww_prod_app",
  }
}

data "aws_ami" "westrikworld" {
  most_recent      = true
  owners           = ["self"]

  filter {
    name   = "name"
    values = ["westrikworld *"]
  }
  filter {
    name   = "tag:Environment"
    values = ["production"]
  }
  filter {
    name   = "tag:OS_Version"
    values = ["Debian 10"]
  }
}

resource "aws_instance" "ww_prod_app" {
  instance_type = "t3a.micro"
  ami = "${data.aws_ami.westrikworld.id}"
  vpc_security_group_ids = ["${aws_security_group.ww_prod_app.id}"]
  subnet_id = "${aws_subnet.ww_prod_app.id}"

  # TODO: change default login and SSH config for AMI (no password)
  # TODO: configure with a stored keypair to allow login via bastion

  tags {
    Name = "ww_prod_app_instance",
    Environment = "production"
  }
}

//resource "aws_eip" "lb" {
//  instance = "${aws_instance.web.id}"
//  vpc      = true
//}




/* ------------------------------
    Configure a VPC, security group, and subnet to build AMIs with
   ------------------------------ */

resource "aws_vpc" "packer_build" {
  cidr_block = "10.0.0.0/16"

  tags = {
    Name = "packer_build"
  }
}

resource "aws_security_group" "packer_build" {
  name        = "packer_build_sg"
  description = "security group for building AMIs with packer"
  vpc_id      = "${aws_vpc.packer_build.id}"

  # SSH access from anywhere
  # TODO: lock down when using CodeBuild for AMI builds
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Outbound HTTPS access (for installing packages)
  # TODO?: specify CIDR block for internal package manager(s)
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Outbound DNS (TCP)
  # TODO?: specify CIDR block for internal DNS server
  egress {
    from_port   = 53
    to_port     = 53
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Outbound DNS (UDP)
  # TODO?: specify CIDR block for internal DNS server
  egress {
    from_port   = 53
    to_port     = 53
    protocol    = "udp"
    cidr_blocks = ["0.0.0.0/0"] # TODO: specify IP for internal DNS
  }
}

resource "aws_internet_gateway" "packer_build" {
  vpc_id = "${aws_vpc.packer_build.id}"
}

resource "aws_route" "packer_build_internet_access" {
  route_table_id         = "${aws_vpc.packer_build.main_route_table_id}"
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = "${aws_internet_gateway.packer_build.id}"
}

resource "aws_subnet" "packer_build" {
  availability_zone       = "us-east-1a"
  vpc_id                  = "${aws_vpc.packer_build.id}"
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true

  tags = {
    "Name" = "packer_build",
    "Network Type" = "Public",
  }
}

/*
# Create an internet gateway to give our subnet access to the outside world
resource "aws_internet_gateway" "ww_prod" {
  vpc_id = "${aws_vpc.ww_prod.id}"
}

# Grant the VPC internet access on its main route table
resource "aws_route" "internet_access" {
  route_table_id         = "${aws_vpc.ww_prod.main_route_table_id}"
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = "${aws_internet_gateway.ww_prod.id}"
}

# Create a subnet to launch our instances into
resource "aws_subnet" "default" {
  availability_zone = "us-east-1a"
  vpc_id                  = "${aws_vpc.ww_prod.id}"
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true
}


# from https://www.terraform.io/docs/providers/tls/r/private_key.html:
# > Important Security Notice
# > The private key generated by this resource will be stored unencrypted in your Terraform state file.
# > Use of this resource for production deployments is not recommended.
# > Instead, generate a private key file outside of Terraform and distribute it securely to the system where Terraform will be run.
resource "tls_private_key" "westrikworld_staging_key" {
  algorithm = "RSA"
  rsa_bits  = 4096
}

resource "aws_key_pair" "generated_key" {
  key_name   = "westrikworld-staging"
  public_key = "${tls_private_key.westrikworld_staging_key.public_key_openssh}"
}

data "aws_ami" "westrikworld" {
  most_recent      = true
  owners           = ["self"]

  filter {
    name   = "name"
    values = ["westrikworld_production *"]
  }
}

resource "aws_instance" "web" {
  # The connection block tells our provisioner how to
  # communicate with the resource (instance)
  connection {
    # The default username for our AMI
    user = "admin"
    host = "${self.public_ip}"
    private_key = "${tls_private_key.westrikworld_staging_key.private_key_pem}"
    # The connection will use the local SSH agent for authentication.
  }

  instance_type = "t3a.micro"

  ami = "${data.aws_ami.westrikworld.id}"

  tags {
    Name = "westrikworld_staging-web"
  }

  # The name of our SSH keypair we generated above.
  key_name = "${aws_key_pair.generated_key.key_name}"

  # Our Security group to allow HTTP and SSH access
  vpc_security_group_ids = ["${aws_security_group.ww_prod.id}"]

  subnet_id = "${aws_subnet.default.id}"
}

resource "aws_eip" "lb" {
  instance = "${aws_instance.web.id}"
  vpc      = true
}
*/
