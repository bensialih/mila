# Mila

## the idea
I wanted to have a dynamic file rotator that does the job based on certain paramters.
What are the parameters? Well, whatever I set them to, and I need these parameters to be dynamic.

## What do I mean by dynamic parameters?

It means that whilst the rotation is running, I want to change the settings of the programme and that it updates how it runs dynamically

## lets talk security
In time, settings can be set in files or in a database.
There is a security component to this, however it is mitigated greatly if you are working with a database, and will need to make accomodations for your file, where it is placed and who can access it.

## how does this differ from other log consolidators like filebeat and fluentd?

I needed something that is dynamic and that can change on the fly.
If there is a better way of doing this, please feel free to contribute.
All ideas and contributions are welcome.
