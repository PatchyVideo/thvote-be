FROM python:3.10-slim

RUN mkdir /app 
COPY . /app
WORKDIR /app
RUN python -m pip install --upgrade pip
RUN pip3 install toml
RUN pip3 install poetry
RUN poetry config virtualenvs.create false
RUN poetry install
RUN sed -i 's/SECLEVEL=2/SECLEVEL=1/g' /etc/ssl/openssl.cnf

CMD ["run", "uvicorn", "main:app", "--port", "80", "--host", "0.0.0.0"]
ENTRYPOINT ["poetry"]
