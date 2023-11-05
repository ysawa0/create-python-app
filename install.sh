#!/bin/bash

USER_REPO="ysawa0/create-python-app"
BINARY_NAME="cpa"
TARGET_DIR="$HOME/bin"

# Determine OS type and corresponding download asset name
OS_TYPE=""
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
	OS_TYPE="Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
	OS_TYPE="macOS"
else
	echo "Unsupported operating system."
	exit 1
fi

# Prepare the binary and zip names
ZIP_NAME="$BINARY_NAME-$OS_TYPE.zip"
ZIP_PATH="$TARGET_DIR/$ZIP_NAME"

# Fetch the latest release data from GitHub API
DOWNLOAD_URL=$(curl -s https://api.github.com/repos/$USER_REPO/releases/latest |
	grep "browser_download_url.*$OS_TYPE.zip" |
	cut -d '"' -f 4)

# Check if the download URL was found
if [ -z "$DOWNLOAD_URL" ]; then
	echo "No release found for $OS_TYPE"
	exit 1
fi

# Create the target directory if it doesn't exist
mkdir -p "$TARGET_DIR"

# Download the zip file
curl -L -o "$ZIP_PATH" "$DOWNLOAD_URL" || {
	echo "Failed to download the release"
	exit 1
}

# Unzip the binary to the target directory
unzip -o "$ZIP_PATH" -d "$TARGET_DIR" || {
	echo "Failed to unzip the release"
	exit 1
}

# Assume the binary is named $BINARY_NAME inside the zip
BINARY_PATH="$TARGET_DIR/$BINARY_NAME"

# Give execute permissions to the binary
chmod +x "$BINARY_PATH" || {
	echo "Failed to set execute permissions on the binary"
	exit 1
}

# Remove the downloaded zip
rm "$ZIP_PATH"

# # Optionally, append the target directory to PATH if it's not already there
# if [[ ":$PATH:" != *":$TARGET_DIR:"* ]]; then
# 	echo "export PATH=\$PATH:$TARGET_DIR" >>~/.bashrc
# 	echo "$TARGET_DIR added to PATH"
# fi

echo "cpa installed to $BINARY_PATH"
