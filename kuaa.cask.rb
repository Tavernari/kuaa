cask "kuaa" do
    version "0.0.1"
    sha256 "86b1cc44cfa93341da90ae63dfc69950d75adbc113cfa795f0d621c970c35cf7"
  
    url "https://github.com/Tavernari/kuaa/releases/download/v#{version}/kuaa-#{version}-macos.tar.gz"
    name "Kuaa"
    desc "CLI tool to enhance Git workflow productivity using AI to craft commits."
    homepage "https://kuaa.tools/"
  
    binary "kuaa"
  end
  