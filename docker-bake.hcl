target "default" {
  context = "."
  dockerfile = "Dockerfile"
  tags = ["composition:latest", "garentyler/composition:latest"]
  platforms = ["linux/amd64", "linux/arm64"]
  target = "prod"
}