# frozen_string_literal: true
require 'toml'

class Builder

  def initialize
    @package_path = './cli/Cargo.toml'
    unless File.file?(@package_path)
      raise "Fail to find file: #{@package_path}"
    end
    @str = File.read(@package_path)
    @toml = TOML::Parser.new(@str).parsed
    @version = @toml['package']['version'] 
  end

  def build
    release_name = "clibri@#{@version}"
    if OS.mac?
      release_name += '-darwin.tgz'
    elsif OS.linux?
      release_name += '-linux.tgz'
    else
      release_name += '-win.tgz'
    end
    Rake.sh "tar -czf ./#{release_name} ./cli/target/release/clibri"
  end

end

