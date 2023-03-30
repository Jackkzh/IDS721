# Project 4 - Serverless Data Engineering Pipeline
This is a project for IDS 721, and the goal of this project is to create a tool for data engineering that uses a big data platform.

## Requirements
* Reproduce the architecture of the example serverless data engineering project or perform something similar using only serverless technologies
* Enhance the project by extending the functionality of the NLP analysis: adding entity extraction, key phrase extraction, or some other NLP feature or doing Applied Computer Vision.

## Project Overview
In this project, I plan to build a serverless pipeline to train and extract information from songs' attributes dateset, which would be .csv files. I plan to use AWS S3 fot data storing, AWS Athena database and AWS Lambda for training. 

## steps 
1. Collect and preprocess data: Create an S3 bucket and upload the dataset to it. Data is collected from Kaggle. Then use AWS Lambda to preprocess the data and store it in Amazon S3.
2. Train the model: Once the data has been preprocessed and stored in Amazon S3, train the recommendation system model using a machine learning framework. I plan to use AWS Lambda, then store the trained model in Amazon S3.
3. - Run queries in AWS Athena and view the songs results, and save them for visualization.



## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)