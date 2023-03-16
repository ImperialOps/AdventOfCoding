######################################
## READ ME LIL BITCH
######################################

resource "kubernetes_namespace" "lil_bitch" {
  metadata {
    name = "test"
  }
}

module "dumb" {
  source  = "terraform-aws-modules/ec2-instance/aws"
  version = "~> 3.0"

  name = "single-instance"

  ami                    = "ami-ebd02392"
  instance_type          = "t2.micro"
  key_name               = "user1"
  monitoring             = true
  vpc_security_group_ids = ["sg-12345678"]
  subnet_id              = "subnet-eddcdzz4"

  tags = {
    Environment = "dev"
    Terraform   = "true"
  }
}

