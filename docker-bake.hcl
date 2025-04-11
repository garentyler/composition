variable "REGISTRY" {
  default = "git.garen.dev/garentyler"
}

variable "GITHUB_SHA" {
  default = "latest"
}

variable "RELEASE_VERSION" {
  default = "latest"
}

target "default" {
  context = "."
  dockerfile = "Dockerfile"
  platforms = ["linux/amd64", "linux/arm64"]
  tags = [
    "${REGISTRY}/composition:${RELEASE_VERSION}",
    "${REGISTRY}/composition:${GITHUB_SHA}"
  ]
  target = "prod"
  args = {
    FEATURES = "server,proxy"
  }
}
