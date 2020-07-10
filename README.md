# conduit-rocket.purs

Conduit is a Medium clone, intended to showcase how to build the same web application using different 
front end and back end frameworks. This particular implementation uses Rust, with Rocket and Diesel as 
the web framework and database interface respectively. 
The API is live at `https://conduit-rocket.herokuapp.com/api/` but doesn't provide a front end.
You can use [this one](https://sylvainleclercq.com/conduit.purs) in order to access it. Simply use the 'Developer'
panel to change the API endpoint to the one given above.

The back-end relies on a free database instance. In order to stay within the limitations imposed by the provider,
the database is wiped regularly.