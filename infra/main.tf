# a lot of this is stolen from: https://github.com/terraform-providers/terraform-provider-aws/tree/master/examples/two-tier

provider "aws" {
  region = "us-east-1"
}

# Create a VPC to launch our instances into
resource "aws_vpc" "default" {
  cidr_block = "10.0.0.0/16"
}

# Create an internet gateway to give our subnet access to the outside world
resource "aws_internet_gateway" "default" {
  vpc_id = "${aws_vpc.default.id}"
}

# Grant the VPC internet access on its main route table
resource "aws_route" "internet_access" {
  route_table_id         = "${aws_vpc.default.main_route_table_id}"
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = "${aws_internet_gateway.default.id}"
}

# Create a subnet to launch our instances into
resource "aws_subnet" "default" {
  availability_zone = "us-east-1a"
  vpc_id                  = "${aws_vpc.default.id}"
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true
}

# Our default security group to access
# the instances over SSH and HTTP
resource "aws_security_group" "default" {
  name        = "tf_example"
  description = "bad, evil security group"
  vpc_id      = "${aws_vpc.default.id}"

  # SSH access from anywhere
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # HTTP access from anywhere
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    # TODO: add an ELB and change cidr_blocks to ["10.0.0.0/16"]
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    # TODO: add an ELB and change cidr_blocks to ["10.0.0.0/16"]
    cidr_blocks = ["0.0.0.0/0"]
  }

  # outbound internet access
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
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
  vpc_security_group_ids = ["${aws_security_group.default.id}"]

  subnet_id = "${aws_subnet.default.id}"

  # We run a remote provisioner on the instance after creating it.
  # In this case, we just install nginx and start it. By default,
  # this should be on port 80
  provisioner "remote-exec" {
    inline = [
      "sudo apt-get -y update",
      "sudo ufw default deny incoming",
      "sudo ufw default allow outgoing",
      "sudo ufw allow ssh",
      "sudo ufw allow 'Nginx Full'",
      "yes | sudo ufw enable",
      "sudo service nginx start",
    ]
  }
}

resource "aws_eip" "lb" {
  instance = "${aws_instance.web.id}"
  vpc      = true
}
