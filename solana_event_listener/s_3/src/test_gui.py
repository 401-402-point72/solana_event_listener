from nicegui import ui
from random import random
import sys
import pandas as pd
import boto3
import json
from botocore.exceptions import ClientError
import logging

logger = logging.getLogger(__name__)

def list_my_buckets(s3_resource):
    print("Buckets:\n\t", *[b.name for b in s3_resource.buckets.all()], sep="\n\t")

# snippet-start:[python.example_code.s3.helper.ObjectWrapper]
class ObjectWrapper:
    """Encapsulates S3 object actions."""

    def __init__(self, s3_object):
        """
        :param s3_object: A Boto3 Object resource. This is a high-level resource in Boto3
                          that wraps object actions in a class-like structure.
        """
        self.object = s3_object
        self.key = self.object.key

    # snippet-start:[python.example_code.s3.GetObject]
    def get(self):
        """
        Gets the object.

        :return: The object data in bytes.
        """
        try:
            body = self.object.get()["Body"].read()
            logger.info(
                "Got object '%s' from bucket '%s'.",
                self.object.key,
                self.object.bucket_name,
            )
        except ClientError:
            logger.exception(
                "Couldn't get object '%s' from bucket '%s'.",
                self.object.key,
                self.object.bucket_name,
            )
            raise
        else:
            return body
    # snippet-end:[python.example_code.s3.GetObject]
        
    # snippet-start:[python.example_code.s3.ListObjects]
    @staticmethod
    def list(bucket, prefix=None):
        """
        Lists the objects in a bucket, optionally filtered by a prefix.

        :param bucket: The bucket to query. This is a Boto3 Bucket resource.
        :param prefix: When specified, only objects that start with this prefix are listed.
        :return: The list of objects.
        """
        try:
            if not prefix:
                objects = list(bucket.objects.all())
            else:
                objects = list(bucket.objects.filter(Prefix=prefix))
            logger.info(
                "Got objects %s from bucket '%s'", [o.key for o in objects], bucket.name
            )
        except ClientError:
            logger.exception("Couldn't get objects for bucket '%s'.", bucket.name)
            raise
        else:
            return objects
    # snippet-end:[python.example_code.s3.ListObjects]

def usage():
    # print("-" * 88)
    # print("Welcome to the Point72 Blockchain GUI demo!")
    # print("-" * 88)

    logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")

    bucket_string = "rustbucketethereum"
    s3_resource = boto3.resource("s3")
    bucket = s3_resource.Bucket(bucket_string)

    listed_lines = ObjectWrapper.list(bucket)
    print(f"Their keys are: {', '.join(l.key for l in listed_lines)}")

    # print(listed_lines[1])
    line =  listed_lines[1]
    line_body = line.get()

    # print(f"Got object with key {line.key} and body(Metadata) {line_body}.")
    # print(line_body['Body'])

    json_data = json.loads(line_body['Body'].read().decode('utf-8'))

    
    df = pd.DataFrame(json_data)
    # print(df)

    time_df = df['timestamp']
    timestamp_value = time_df.at[0]
    # print(timestamp_value)

    blocknum_df = df['currentBlockNumber']
    block_value = blocknum_df.at[0]

    blockHash_df = df['blockHash']
    hash_value = blockHash_df.at[0]

    gasUsed_df = df['gasUsed']
    gasUsed_value = gasUsed_df.at[0]

    gasLimit_df = df['gasLimit']
    gasLimit_value = gasLimit_df.at[0]

    transactions_df = pd.DataFrame(df['transactions'])
    # print(transactions_df)
    transactions_df = transactions_df.explode('transactions')

    # Convert each dictionary in the 'transactions' column to a DataFrame
    transactions_df = pd.json_normalize(transactions_df['transactions'])

    # Convert integer columns to strings
    transactions_df['value'] = transactions_df['value'].astype(str)

    # print(transactions_df)

    dark = ui.dark_mode()
    dark.enable()

    with ui.tabs().classes('w-full h-full') as tabs:
        eth = ui.tab('Ethereum')
        sol = ui.tab('Solana')

    with ui.tab_panels(tabs, value=eth).classes('w-full h-full'):
        with ui.tab_panel(eth):
            ui.tree([
                {'id': 'Block Data', 'children': [
                    {'id': 'Timestamp', 'children': [ {'id': timestamp_value} ]},
                    {'id': 'Current Block Number' , 'children': [ {'id': block_value} ]}, 
                    {'id': 'Block Hash' , 'children': [ {'id': hash_value} ]}, 
                    {'id': 'Gas Used' , 'children': [ {'id': gasUsed_value} ]}, 
                    {'id': 'gasLimit' , 'children': [ {'id': gasLimit_value} ]},
                ]},
            ],  label_key='id' )
            # label_key='id', on_select=lambda  e: ui.notify(e.value) )

            ui.separator()
            ui.markdown('# Transactions')

            columns = [
                {'name': 'transactionHash', 'label': 'Transaction Hash', 'field': 'transactionHash', 'required': True, 'align': 'left'},
                {'name': 'from', 'label': 'From', 'field': 'from', 'align': 'left'},
                {'name': 'to', 'label': 'To', 'field': 'to', 'align': 'left'},
                {'name': 'value', 'label': 'Value', 'field': 'value', 'align': 'left'}
            ]

   
            rows = transactions_df.to_dict(orient='records')

            ui.table(columns=columns, rows=rows, row_key='transactionHash', )
            # with ui.scroll_area().classes('w-3200 h-5000 border'):
            #     # Display the table
            #     ui.table(columns=columns, rows=rows, row_key='transactionHash', )


        with ui.tab_panel(sol):
            ui.label('Solana Data')

    



    ui.run()



if __name__ in {"__main__", "__mp_main__"}:

    usage()


