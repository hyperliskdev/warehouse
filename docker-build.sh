#!/bin/sh


build_service_image() {
  echo "Building $1 image..."
}


## Main Menu
dialog --menu "Select an image to build..." 30 30 12 \
  1 "services" \
  2 "router" 2> "/tmp/choice.txt"

service=$(cat /tmp/choice.txt)


case $service in 
  1) build_service_image(${service}) ;;
  2) echo "Building HRMS image..." ;;
  *) echo "Invalid selection" ;;
esac