#!/bin/ruby

module LineRead
  module_function
  def run
    File.readlines("steevo.txt").sample
  end
end

post = LineRead.run.chomp
system("toot activate Steevo@welovela.in")
system("toot post \"cucked again by @lunarised :\(\"")
system("echo "+post+"");
