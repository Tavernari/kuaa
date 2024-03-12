class Kuaa < Formula
    desc "CLI tool to enhance Git workflow productivity using AI to craft commits."
    homepage "https://kuaa.tools"
    url "https://github.com/Tavernari/kuaa/archive/refs/tags/0.0.1.tar.gz"
    sha256 "86b1cc44cfa93341da90ae63dfc69950d75adbc113cfa795f0d621c970c35cf7"
  
    depends_on "rust" => :build
  
    def install
      system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    end
  
    test do
      system "#{bin}/kuaa", "--version"
    end
  end
  