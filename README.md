# mischief

just started up.
goal is send json to endpoint, it can do stuff like

* create/delete/rename/permchange files
* download files from the internet
* create users
* 'browse the web'
* send email
* other stuff because feature creep is the best feature

```
nc localhost 8080
{"name":"filesystem", "data":"howdy partner", "counter":9}
14


{"taskId":22,"commandType":1,"function":"create_file","iterations":2,"params":{"filename":"testing.txt","block":"yes"}}

```
