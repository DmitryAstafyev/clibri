require 'fileutils'
require './rakefile.module.builder'
require './rakefile.module.env'

module PATHS
  self::CLI = "./cli"
  self::LIB = "./lib"
  self::PROTOCOL_TEST = "./tests/protocol"
  self::WORKFLOW_TEST = "./tests/workflow"
  self::EXAMPLES = "./examples"
  self::TRANSPORT = "./environment/transport"
end

namespace :cli do
  desc 'Build CLI'
  task :build do
    Dir.chdir(PATHS::CLI) do
      sh 'cargo build --release'
    end
  end
end

namespace :lib do

  namespace :build do
    desc 'Build Rust Lib'
    task :rs do
      Dir.chdir("#{PATHS::LIB}/rust") do
        sh 'cargo build --release'
      end
    end
    desc 'Build Typescript Lib'
    task :ts do
      Dir.chdir("#{PATHS::LIB}/typescript") do
        sh 'npm install'
        sh 'npm run build'
      end
    end
    task :all => ['rs', 'ts']
  end

end

namespace :transport do
  namespace :rs do
    desc 'Build Rust Server'
    task :server do
      Dir.chdir("#{PATHS::TRANSPORT}/server/rust") do
        sh 'cargo build --release'
      end
    end
    task :client do
      Dir.chdir("#{PATHS::TRANSPORT}/client/rust") do
        sh 'cargo build --release'
      end
    end
    desc 'Build Server & Client'
    task :build => ['server', 'client']
  end
  namespace :ts do
    desc 'Build Rust Server'
    task :server do
      Dir.chdir("#{PATHS::TRANSPORT}/server/typescript") do
        sh 'npm install'
        sh 'npm run build'
      end
    end
    task :client do
      Dir.chdir("#{PATHS::TRANSPORT}/client/typescript") do
        sh 'npm install'
        sh 'npm run build'
      end
    end
    desc 'Build Server & Client'
    task :build => ['server', 'client']
  end
  desc 'Build All'
  task :build => ['rs:build', 'ts:build']
end

namespace :test do
  namespace :protocol do 
    desc 'Generate Test Protocols'
    task :generate do
      sh "#{PATHS::CLI}/target/release/clibri --src #{PATHS::PROTOCOL_TEST}/prot/protocol.prot -rs #{PATHS::PROTOCOL_TEST}/rust/src/protocol.rs -ts #{PATHS::PROTOCOL_TEST}/typescript/src/protocol.ts -o --em"
    end
  
    desc 'Build Protocol Test Executors'
    task :build do
      Dir.chdir("#{PATHS::PROTOCOL_TEST}/rust") do
        sh 'cargo build --release'
      end
      Dir.chdir("#{PATHS::PROTOCOL_TEST}/typescript") do
        sh 'npm install'
        sh 'npm run build'
      end
    end
  
    desc 'Executing Tests'
    task :execute do
      errors = []
      Dir.chdir("#{PATHS::PROTOCOL_TEST}/typescript") do
        begin
          sh 'node ./dist/index.js write'
        rescue StandardError => e
          errors << e
        end
      end
      Dir.chdir("#{PATHS::PROTOCOL_TEST}/rust") do
        begin
          sh './target/release/clibri_protocol_rust_test write'
        rescue StandardError => e
          errors << e
        end
      end
      Dir.chdir("#{PATHS::PROTOCOL_TEST}/typescript") do
        begin
          sh 'node ./dist/index.js read'
        rescue StandardError => e
          errors << e
        end
      end
      Dir.chdir("#{PATHS::PROTOCOL_TEST}/rust") do
        begin
          sh './target/release/clibri_protocol_rust_test read'
        rescue StandardError => e
          errors << e
        end
      end
      es = errors.reduce('') { |acc, e| [acc, e].join('\n') }
      raise es unless errors.empty?
    end
  
    desc 'Test'
    task :all => ['cli:build', 'generate', 'build', 'execute']
  
  end

  namespace :examples do
    desc 'Producer - rust / Consumer - rust'
    task :rs_rs do
      Dir.chdir("#{PATHS::EXAMPLES}") do
        sh '../cli/target/release/clibri -s ./prot/protocol.prot -wf ./prot/protocol-rs-rs.workflow -cd ./consumer/rust/src/consumer/ -pd ./producer/rust/src/producer/'
      end
      Dir.chdir("#{PATHS::EXAMPLES}/producer/rust") do
        sh 'cargo build --release'
      end
      Dir.chdir("#{PATHS::EXAMPLES}/consumer/rust") do
        sh 'cargo build --release'
      end
    end

    desc 'Producer - typescript / Consumer - typescript'
    task :ts_ts do
      Dir.chdir("#{PATHS::EXAMPLES}") do
        sh '../cli/target/release/clibri -s ./prot/protocol.prot -wf ./prot/protocol-ts-ts.workflow -cd ./consumer/typescript/src/consumer/ -pd ./producer/typescript/src/producer/'
      end
      Dir.chdir("#{PATHS::EXAMPLES}/producer/typescript") do
        sh 'npm install'
        sh 'npm run build'
      end
      Dir.chdir("#{PATHS::EXAMPLES}/consumer/typescript") do
        sh 'npm install'
        sh 'npm run build'
      end
    end

    desc 'Producer - rust / Consumer - typescript'
    task :rs_ts do
      Dir.chdir("#{PATHS::EXAMPLES}") do
        sh '../cli/target/release/clibri -s ./prot/protocol.prot -wf ./prot/protocol-rs-ts.workflow -cd ./consumer/typescript/src/consumer/ -pd ./producer/rust/src/producer/'
      end
      Dir.chdir("#{PATHS::EXAMPLES}/producer/rust") do
        sh 'cargo build --release'
      end
      Dir.chdir("#{PATHS::EXAMPLES}/consumer/typescript") do
        sh 'npm install'
        sh 'npm run build'
      end
    end

    desc 'Create All'
    task :create => ['rs_rs', 'ts_ts', 'rs_ts']

  end

  namespace :workflow do
    desc 'Rust test executor'
    task :rs_executor do
      Dir.chdir("#{PATHS::WORKFLOW_TEST}/executor") do
        sh 'cargo build --release'
      end
      Dir.chdir("#{PATHS::WORKFLOW_TEST}") do
        sh './executor/target/release/clibri_test_executor --producer=./run-producer-rs-gitactions.sh --consumer=./run-consumer-rs-gitactions.sh'
      end
    end
    desc 'Producer - rust / Consumer - rust'
    task :rs_rs do
      Dir.chdir("#{PATHS::WORKFLOW_TEST}") do
        sh '../../cli/target/release/clibri -s ./prot/protocol.prot -wf ./prot/protocol-rs-rs.workflow -cd ./consumer/rust/src/consumer/ -pd ./producer/rust/src/producer/'
      end
      Dir.chdir("#{PATHS::WORKFLOW_TEST}/producer/rust") do
        sh 'cargo build --release'
      end
      Dir.chdir("#{PATHS::WORKFLOW_TEST}/consumer/rust") do
        sh 'cargo build --release'
      end
    end
    desc "Test Producer/Consumer - rust"
    task :rs_test => ['rs_rs', 'rs_executor']

    desc 'TS test executor'
    task :ts_executor do
      Dir.chdir("#{PATHS::WORKFLOW_TEST}/executor") do
        sh 'cargo build --release'
      end
      Dir.chdir("#{PATHS::WORKFLOW_TEST}") do
        sh './executor/target/release/clibri_test_executor --node --producer=./producer/typescript/dist/index.js --consumer=./consumer/typescript/dist/index.js'
      end
    end
    desc 'Producer - typescript / Consumer - typescript'
    task :ts_ts do
      Dir.chdir("#{PATHS::WORKFLOW_TEST}") do
        sh '../../cli/target/release/clibri -s ./prot/protocol.prot -wf ./prot/protocol-ts-ts.workflow -cd ./consumer/typescript/src/consumer/ -pd ./producer/typescript/src/producer/'
      end
      Dir.chdir("#{PATHS::WORKFLOW_TEST}/producer/typescript") do
        sh 'npm install'
        sh 'npm run build'
      end
      Dir.chdir("#{PATHS::WORKFLOW_TEST}/consumer/typescript") do
        sh 'npm install'
        sh 'npm run build'
      end
    end
    desc "Test Producer/Consumer - rust"
    task :ts_test => ['ts_ts', 'ts_executor']

    desc 'Test All'
    task :all => ['rs_test', 'ts_test']
  end

  desc 'Test All'
  task :all => ['cli:build', 'test:protocol:all', 'test:workflow:all', 'test:examples:create']
    
end

namespace :clean do
  desc 'Clean Lib'
  task :lib do
    rm_rf("#{PATHS::LIB}/rust/target", verbose: true)
    rm_rf("#{PATHS::LIB}/typescript/node_modules", verbose: true)
    rm_rf("#{PATHS::LIB}/typescript/dist", verbose: true)
    # rm("#{PATHS::LIB}/typescript/package-lock.json", verbose: true) unless !Dir.exist?("#{PATHS::LIB}/typescript/package-lock.json")
  end
  task :cli do
    rm_rf("#{PATHS::CLI}/target", verbose: true)
  end
  task :examples do
    rm_rf("#{PATHS::EXAMPLES}/producer/rust/target", verbose: true)
    rm_rf("#{PATHS::EXAMPLES}/consumer/rust/target", verbose: true)
    rm_rf("#{PATHS::EXAMPLES}/producer/typescript/node_modules", verbose: true)
    rm_rf("#{PATHS::EXAMPLES}/producer/typescript/dist", verbose: true)
    rm_rf("#{PATHS::EXAMPLES}/consumer/typescript/node_modules", verbose: true)
    rm_rf("#{PATHS::EXAMPLES}/consumer/typescript/dist", verbose: true)
  end
  task :transport do
    rm_rf("#{PATHS::TRANSPORT}/server/rust/target", verbose: true)
    rm_rf("#{PATHS::TRANSPORT}/client/rust/target", verbose: true)
    rm_rf("#{PATHS::TRANSPORT}/server/typescript/node_modules", verbose: true)
    rm_rf("#{PATHS::TRANSPORT}/server/typescript/dist", verbose: true)
    rm_rf("#{PATHS::TRANSPORT}/client/typescript/node_modules", verbose: true)
    rm_rf("#{PATHS::TRANSPORT}/client/typescript/dist", verbose: true)
  end
  task :protocol_test do
    rm_rf("#{PATHS::PROTOCOL_TEST}/rust/target", verbose: true)
    rm_rf("#{PATHS::PROTOCOL_TEST}/rust/binary", verbose: true)
    rm_rf("#{PATHS::PROTOCOL_TEST}/typescript/node_modules", verbose: true)
    rm_rf("#{PATHS::PROTOCOL_TEST}/typescript/dist", verbose: true)
    rm_rf("#{PATHS::PROTOCOL_TEST}/typescript/binary", verbose: true)
  end
  task :all => ['lib', 'cli', 'examples', 'transport', 'protocol_test']

end

desc 'Release CLI'
task :release do
  @builder = Builder.new
  @builder.build
end