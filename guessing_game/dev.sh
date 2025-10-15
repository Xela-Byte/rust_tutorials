#!/usr/bin/env bash

# 🦀 Rust Dev Runner (Nodemon-style)
# ---------------------------------
# Features:
# - Type "rs" to restart manually
# - Type "q" to quit
# - Colorful logs with timestamps
# - Clears terminal on restart

# Colors
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
RED="\033[1;31m"
CYAN="\033[1;36m"
RESET="\033[0m"

clear
echo -e "${CYAN}🦀 Rust Dev Watcher Started${RESET}"
echo -e "${YELLOW}Type 'rs' to restart, or 'q' to quit.${RESET}\n"

while true; do
  echo -e "${CYAN}[$(date '+%H:%M:%S')] Running cargo...${RESET}"
  echo "-------------------------------------"
  cargo watch -q -c -x run
  echo "-------------------------------------"
  echo -e "${GREEN}[$(date '+%H:%M:%S')] Process finished.${RESET}"
  echo
  echo -e "${YELLOW}Type 'rs' to restart, or 'q' to quit.${RESET}"

  read -r input
  case "$input" in
    rs)
      clear
      echo -e "${CYAN}🔁 Restarting... $(date '+%H:%M:%S')${RESET}\n"
      ;;
    q)
      echo -e "${RED}👋 Exiting Rust Dev Watcher.${RESET}"
      break
      ;;
    *)
      echo -e "${RED}❓ Unknown command:${RESET} '$input'"
      ;;
  esac
done
