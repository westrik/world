// - Set up a VPC, security group, and subnet for our application servers to live in

/*
TODO(later):
  - [ ] handle IPv6
  - [ ] provision ACM private cert to use with NLB
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

resource "aws_security_group" "app_inbound" {
  name        = "app_in_sg"
  description = "Primary ${var.project_name} production SG (inbound)"
  vpc_id      = aws_vpc.app.id

  # SSH access from me
  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    //    cidr_blocks = ["10.0.0.0/16"]
  }

  # Inbound HTTP via NLB
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  # Inbound HTTPS via NLB
  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

}
resource "aws_security_group" "app_outbound_s3" {
  name        = "app_out_s3_sg"
  description = "Primary ${var.project_name} production SG (outbound for s3)"
  vpc_id      = aws_vpc.app.id

  # Outbound HTTPS access to S3 (via VPC endpoint)
  egress {
    from_port = 443
    to_port   = 443
    protocol  = "tcp"
    prefix_list_ids = [
    aws_vpc_endpoint.s3.prefix_list_id]
  }

  # Outbound DNS access TODO: VPC endpoint?
  egress {
    from_port = 53
    to_port   = 53
    protocol  = "tcp"
    cidr_blocks = [
    "0.0.0.0/0"]
    # TODO: specify CIDR for DNS
  }
}


data "aws_ip_ranges" "amazon_services" {
  regions  = [var.aws_region]
  services = ["amazon"]
}

resource "aws_security_group" "app_outbound" {
  name        = "app_out_sg"
  description = "Primary ${var.project_name} production SG (outbound)"
  vpc_id      = aws_vpc.app.id

  # Outbound HTTPS to AWS (CodeDeploy)
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] #data.aws_ip_ranges.amazon_services.cidr_blocks
  }

  # RDS
  egress {
    from_port = 5432
    to_port   = 5432
    protocol  = "tcp"
    cidr_blocks = [
    "10.0.0.0/16"]
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
