#!/bin/bash
# Java stack (OpenJDK 21 via apt, Gradle via SDKMAN)
set -e

apt-get update
apt-get install -y openjdk-21-jdk

source /root/.sdkman/bin/sdkman-init.sh
sdk install gradle 8.10.2

# Create a combined profile script
cat > /etc/profile.d/java.sh << 'EOF'
export JAVA_HOME=/usr/lib/jvm/java-21-openjdk-arm64
export GRADLE_HOME=/root/.sdkman/candidates/gradle/current
export PATH=$JAVA_HOME/bin:$GRADLE_HOME/bin:$PATH
source /root/.sdkman/bin/sdkman-init.sh
EOF

apt-get clean
rm -rf /var/lib/apt/lists/*

echo "Java stack installed: $(/usr/lib/jvm/java-21-openjdk-amd64/bin/java -version 2>&1 | head -1)"
echo "Gradle installed: $(gradle --version 2>&1 | head -1)"