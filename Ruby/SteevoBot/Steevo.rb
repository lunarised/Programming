#!/bin/ruby

module LineRead
  module_function
  def run
    File.readlines("steevo.txt").sample
  end
end

post = LineRead.run.chomp
system("toot post \""+ post +"\"")
