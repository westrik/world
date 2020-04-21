// - Set up a VPC, security group, and subnet for our application servers to live in

/*
TODO:
  - [ ] handle IPv6
*/

provider "aws" {
  region = var.aws_region
}

resource "aws_vpc" "app" {
  cidr_block = "10.0.0.0/16"

  tags = {
    Name        = "app_vpc"
    Environment = "production"
  }
}

resource "aws_vpc_endpoint" "s3" {
  vpc_id          = aws_vpc.app.id
  service_name    = "com.amazonaws.${var.aws_region}.s3"
  route_table_ids = [aws_vpc.app.main_route_table_id]

  tags = {
    Environment = "production"
  }
}


resource "aws_internet_gateway" "app" {
  vpc_id = aws_vpc.app.id
}

resource "aws_route" "app_internet_access" {
  route_table_id         = aws_vpc.app.main_route_table_id
  gateway_id             = aws_internet_gateway.app.id
  destination_cidr_block = "0.0.0.0/0"
}

resource "aws_subnet" "app_az1" {
  availability_zone       = var.aws_az1
  vpc_id                  = aws_vpc.app.id
  cidr_block              = "10.0.1.0/24"
  map_public_ip_on_launch = true

  tags = {
    Name        = "app"
    Environment = "production"
  }
}

resource "aws_subnet" "app_az2" {
  availability_zone = var.aws_az2
  vpc_id            = aws_vpc.app.id
  cidr_block        = "10.0.2.0/24"

  tags = {
    Name        = "app"
    Environment = "production"
  }
}

module "security_groups" {
  source = "./security_groups"

  project_name    = var.project_name
  vpc_id          = aws_vpc.app.id
  prefix_list_ids = [aws_vpc_endpoint.s3.prefix_list_id]
}
