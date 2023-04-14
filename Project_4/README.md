# Project 4 - Serverless Data Engineering Pipeline
This is a project for IDS 721, and the goal of this project is to create a tool for data engineering that uses a big data platform.

## Requirements
* Reproduce the architecture of the example serverless data engineering project or perform something similar using only serverless technologies
* Enhance the project by extending the functionality of the NLP analysis: adding entity extraction, key phrase extraction, or some other NLP feature or doing Applied Computer Vision.

## Project Overview
In this project, I plan to build a serverless pipeline to train and extract information from songs' attributes dateset, which would be .csv files. I plan to use AWS S3 fot data storing, AWS Athena database and AWS Lambda for training. 

![image](https://user-images.githubusercontent.com/101923398/231911786-de90a278-74c3-4908-80e0-40641f7e35fd.png)

Store Spotify music data in an S3 bucket. During this process, you can divide the data into multiple CSV files and upload them to an S3 bucket.

Create a Lambda function that will perform music data analysis on the data in S3 and store the results in another S3 bucket.

Using the Athena query service, you can use SQL query language to query data in the S3 bucket. You can use Athena query language to filter data, group data, compute data, and more.

Create a Lambda function that will perform necessary tasks based on your requirements, such as storing data in a database or sending results to an SQS queue.


## steps 
1. Collect and preprocess data: Create an S3 bucket and upload the dataset to it. Data is collected from Kaggle. Then use AWS Lambda to preprocess the data and store it in Amazon S3.
2. Train the model: Once the data has been preprocessed and stored in Amazon S3, train the recommendation system model using a machine learning framework. I plan to use AWS Lambda, then store the trained model in Amazon S3.
3. Run queries in AWS Athena and view the songs results, and save them for visualization.



## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* https://github.com/noahgift/awslambda
