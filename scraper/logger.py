from os import path, mkdir

from loguru import logger

cwd = path.abspath(path.dirname(__file__))
logs_dir = path.join(cwd, 'logs')
if not path.isdir(logs_dir):
    mkdir(logs_dir)


logger.add(path.join(logs_dir, "error.{time:YYYY-MM-DD}.log"),
           rotation="00:00",
           diagnose=False,
           level="ERROR",
           encoding='utf-8',
           retention='3 days')

logger.add(path.join(logs_dir, "debug.{time:YYYY-MM-DD}.log"),
           rotation="00:00",
           diagnose=False,
           level="DEBUG",
           encoding='utf-8',
           retention='3 days')

logger.add(path.join(logs_dir, "info.{time:YYYY-MM-DD}.log"),
           rotation="00:00",
           diagnose=False,
           level="INFO",
           encoding='utf-8',
           retention='7 days')
