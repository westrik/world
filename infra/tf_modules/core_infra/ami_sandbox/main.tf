// Configure a VPC, security group, and subnet to build AMIs with

provider "aws" {
  region = var.aws_region
}

resource "aws_vpc" "packer_build" {
  cidr_block = "10.0.0.0/16"

  tags = {
    Name = "packer_build"
  }
}

resource "aws_security_group" "packer_build" {
  name        = "packer_build_sg"
  description = "security group for building AMIs with packer"
  vpc_id      = aws_vpc.packer_build.id

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
  vpc_id = aws_vpc.packer_build.id
}

resource "aws_route" "packer_build_internet_access" {
  route_table_id         = aws_vpc.packer_build.main_route_table_id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.packer_build.id
}

resource "aws_subnet" "packer_build" {
  availability_zone       = var.aws_az
  vpc_id                  = aws_vpc.packer_build.id
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true

  tags = {
    "Name"         = "packer_build"
    "Network Type" = "Public"
  }
}
