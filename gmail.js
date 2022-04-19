var nodemailer = require('nodemailer');

		var transporter = nodemailer.createTransport({
		  service: 'gmail',
		  auth: {
		    user: 'nicklegato@gmail.com',
		    pass: '******************'
		  }
		});

		var mailOptions = {
		  from: 'nicklegato@gmail.com',
		  to: 'nicklegato@gmail.com',
		  subject: 'finally',
		  text: 'it works $F'
		};

		transporter.sendMail(mailOptions, function(error, info){
		  if (error) {
		    console.log(error);
		  } else {
		    console.log('Email sent: ' + info.response);
		  }
		});