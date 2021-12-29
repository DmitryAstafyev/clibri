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
      Rake.sh "tar -czf ./#{release_name} ./cli/target/release/clibri"
    elsif OS.linux?
      release_name += '-linux.tgz'
      Rake.sh "tar -czf ./#{release_name} ./cli/target/release/clibri"
    else
      release_name += '-win.tgz'
      Rake.sh "tar -cvzf ./#{release_name} ./cli/target/release/clibri.exe"
    end
  end

end

