terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

provider "aws" {
  region  = "eu-west-1"
  profile = "rickard"


}

resource "aws_s3_bucket" "frontend_bucket" {
  bucket_prefix = "nian-rust-"

  acl = "public-read"
}

data "archive_file" "init" {
  type        = "zip"
  source_dir  = var.files
  output_path = "output.zip"
}

resource "null_resource" "web_upload" {
  triggers = {
    src_hash = data.archive_file.init.output_sha
  }
  provisioner "local-exec" {
    command = "aws --profile rickard s3 sync ${var.files} s3://${aws_s3_bucket.frontend_bucket.bucket} --acl public-read --delete"
  }

  provisioner "local-exec" {
    command = "aws --profile rickard cloudfront create-invalidation --distribution-id ${aws_cloudfront_distribution.s3_distribution.id} --paths '/*'"
  }
}

resource "aws_cloudfront_origin_access_identity" "my_access_identity" {
  comment = "access-identity-nian-rust"
}

resource "aws_cloudfront_distribution" "s3_distribution" {
  origin {
    domain_name = aws_s3_bucket.frontend_bucket.bucket_regional_domain_name
    origin_id   = aws_cloudfront_origin_access_identity.my_access_identity.id

    s3_origin_config {
      origin_access_identity = aws_cloudfront_origin_access_identity.my_access_identity.cloudfront_access_identity_path
    }
  }

  default_cache_behavior {
    allowed_methods = [
      "DELETE",
      "GET",
      "HEAD",
      "OPTIONS",
      "PATCH",
      "POST",
    "PUT"]
    cached_methods = [
      "GET",
    "HEAD"]
    target_origin_id = aws_cloudfront_origin_access_identity.my_access_identity.id

    forwarded_values {
      query_string = false

      cookies {
        forward = "none"
      }
    }

    viewer_protocol_policy = "redirect-to-https"
    min_ttl                = 0
    default_ttl            = 3600
    max_ttl                = 84000
  }

  enabled             = true
  default_root_object = "index.html"

  viewer_certificate {
    cloudfront_default_certificate = true
  }


  #   logging_config {
  #     include_cookies = false
  #     bucket          = aws_s3_bucket.log-bucket.bucket_domain_name
  #   }

  price_class = "PriceClass_100"
  restrictions {
    geo_restriction {
      restriction_type = "whitelist"
      locations        = ["SE", "GB"]
    }
  }
}


