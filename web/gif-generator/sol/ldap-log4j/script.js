const ldap = require('ldapjs');

const server = ldap.createServer();
// specify the port that the LDAP server will listen to
const PORT = 4000;

server.bind('', function(req, res, next) {
  var obj = {
    dn: req.dn.toString(),
    attributes: {
      objectclass: ['organization', 'top'],
      o: 'example'
    }
  };

  if (req.filter.matches(obj.attributes))
  res.send(obj);

  res.end();
});

// Start the server
server.listen(PORT, '0.0.0.0', function() {
  console.log('LDAP server listening at: ' + server.url);
});
