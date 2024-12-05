variable "TAG" {
  default = "latest"
}

target "default" {
  context = "."
  dockerfile = "Dockerfile"
  platforms = ["linux/amd64", "linux/arm64"]
  tags = ["composition:${TAG}", "garentyler/composition:${TAG}"]
  target = "prod"
  args = {
    FEATURES = "server,proxy"
  }
}
