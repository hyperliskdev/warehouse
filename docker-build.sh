#!/bin/bash

configure_service_def() {
  echo "Configuring service definition..."

  service=$(dialog --clear --backtitle "Warehouse Docker Builder" \
      --title "warehouse" \
      --menu "Enter service name:" \
      20 20 2 \
      2>&1>$(tty))

}

build_service_image() {
  echo "Building $1 image..."
}


## Main Menu
image_choice=$(dialog --clear --backtitle "Warehouse Docker Builder" \
			--title "warehouse" \
			--menu "Choose an option:" \
			20 20 2 \
			1 "Services" \
      2 "Router" \
			2>&1>$(tty))


case $image_choice in 
  1) configure_service_def ;;
  2) configure_router_def ;;
  *) echo "Invalid selection" ;;
esac