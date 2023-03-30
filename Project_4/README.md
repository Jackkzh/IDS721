# Rust Big Data Platform Project
This is a project for IDS 721, and the goal of this project is to create a tool for data engineering that uses a big data platform.

## Project Overview
I would practice using SQL-like queries to analyze large datasets with AWS Athena. The dataset that I would use is the NYC taxi trip dataset, which is a popular open dataset of taxi trips in New York City.

## Steps to use AWS Athena
- Create an S3 bucket and upload the dataset to it. I would either use the AWS S3 console or the AWS CLI to do this.
- Create an Athena database and table to store the dataset (using the AWS Athena console or the AWS CLI). Specify the correct file format and schema for the dataset.
- Write SQL queries to analyze the data (in AWS Athena console or in a text editor). Test queries and make sure they produce the desired results.
- Run queries in AWS Athena and view the results (AWS Athena console or the AWS CLI). Save results to a file for further analysis or visualization.

## Rust parts
- preprocess the data before it is queried in Athena
- clean, filter, or transform the data before uploading to Amazon S3

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)