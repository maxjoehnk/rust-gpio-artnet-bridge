#!/usr/bin/env groovy
pipeline {
    agent {
    	dockerfile {
    		filename '.ci/Dockerfile
		}
	}
    options {
        buildDiscarder(logRotator(artifactNumToKeepStr: '3'))
    }
    stages {
        stage('Build') {
            steps {
                checkout scm
                sh 'cargo build --target=arm-unknown-linux-gnueabihf'
				archive 'arm-unknown-linux-gnueabihf/debug/rust-gpio-artnet-bridge'
            }
        }
    }
}
