# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/trusty64"

  config.vm.synced_folder ".", "/cloudwrap", rsync: "true"

  config.vm.provision "shell", inline: <<-SHELL
    command -v cargo >/dev/null 2>&1 && exit 0;

    wget -O rustup.sh https://sh.rustup.rs
    sh rustup.sh -y
    rm -f rustup.sh
  SHELL

  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get --assume-yes install dpkg-dev
    apt-get --assume-yes install lintian
    apt-get --assume-yes install debhelper
    apt-get --assume-yes install pkg-config
    apt-get --assume-yes install autoconf
    apt-get --assume-yes install libtool
    apt-get --assume-yes install openssl
    apt-get --assume-yes install libssl-dev
    apt-get --assume-yes install ruby
    apt-get --assume-yes install ruby-dev
    apt-get --assume-yes install rubygems
    apt-get --assume-yes install build-essential
    apt-get --assume-yes install git
  SHELL

  config.vm.provision "shell", inline: <<-SHELL
    rustup target add x86_64-unknown-linux-musl
    gem install --no-ri --no-rdoc fpm
  SHELL
end
