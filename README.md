# Command Prompt Emailer
This command line project was created to send an email, using a gmail account, from the Windows command prompt. Rust doesn't really have a
way to do it by itself so, I decided to use nodemailer and Node.js for sending. Maybe in the future, I will do it in straight rust.
Maybe even create a crate for the public, but for now this was just for learning purposes. Basicaly, rust gets the user's input and data then
writes a javascript file with the information accuired. Rust then runs Node.js to serve the file. Anyone is more then welcome to use or modify this code.
I'm not using it for enterprise purposes so have at it!

## Features
- Contact list that can be modified
- Help menu
- Send an email to someone using your gmail

## How to Use
First thing you need to do if you have not is <a href ="https://www.rust-lang.org/tools/install">Install Rust</a>.

The next is intall Node.js & npm by downloading the installer at <a href="https://nodejs.org/en/download/.">Node.js</a>.
        
Then install nodemailer

        C:\Users\Your Name>npm install nodemailer
        
Now that everything is installed clone this repository then open cmd go to your folder with this project
      
      C:\Users\this project>cargo run "something" -h
      
If everything went well, you should now see the the help menu. It will show you the way. :)

ps....make sure you have less secure in gmail enabled. Or you will be blocked!!!

## Lessons Learned
- Plan the project as much as possible before coding. As I built this project the layout and pattern changed, do to features that I did not account for 
in planning. This wasn't to bad but it really did create more work that could have been avoided.
- Why using traits, generics, & lifetimes in Rust actually make programs easier to modify and resuse later down the road.
- File manipulation. I had to create a js file while inputing certain information. And when I created a contacts list that saves structs in a vec then saves 
it in a file. Anytime, it is modified it is pulled back into it's code form. For those of you that don't know. This is also refered to as serailization.

## Problems
Two problems I encounterd was editing the message in the cmd. I could clear it all the way and delete the line I was on, but going back the line before was an
issue that I'm still working around. The second problem, which I will fix in the future, is that Node.js does'nt like escape characters in rust. Such as 
\r or \n. It created errors with Node, so only one liners for now! ;)
