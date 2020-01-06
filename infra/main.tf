/*
TODO:
  - set up NLB with Elastic IP, point at EC2 box

  - set up RDS
  - set up IAM roles so things can talk properly
  - set up CodeDeploy to EC2 instances
  - set up S3
  - set up CloudFront
  - provision ACM private cert to use with NLB
  - set up new IAM role for scripted use (i.e. not root account)
  - handle IPv6
*/

provider "aws" {
  region = "${var.aws_region}"
}

/*
---------------------------------------------------------------------
Set up VPC, security group, subnet, EC2 and RDS instances, and an NLB
---------------------------------------------------------------------
*/

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
  availability_zone = "${var.aws_az}"
  vpc_id                  = "${aws_vpc.ww_prod_app.id}"
  cidr_block              = "10.0.1.0/24"

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

// domain_name = "${var.api_domain_name}"
// domain_name = "${var.frontend_domain_name}"



/*
---------------------------------------------------------------------
 Configure a VPC, security group, and subnet to build AMIs with
---------------------------------------------------------------------
*/

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
  availability_zone       = "${var.aws_az}"
  vpc_id                  = "${aws_vpc.packer_build.id}"
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true

  tags = {
    "Name" = "packer_build",
    "Network Type" = "Public",
  }
}
