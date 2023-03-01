FROM python:3.8.10
RUN mkdir /Project_2
WORKDIR /Project_2
COPY . /Project_2
RUN pip3 install -r requirements.txt
EXPOSE 8000
CMD ["python3", "app.py"]