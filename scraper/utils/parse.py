import re
from html import unescape


def html_to_plain_text(html: str):
    text = re.sub('<head.*?>.*?</head>', '', html, flags=re.M | re.S | re.I)
    text = re.sub('<a\s.*?>', '', text, flags=re.M | re.S | re.I)
    text = re.sub('<br.*?>', '\n', text, flags=re.M | re.S)
    text = re.sub('<.*?>', '', text, flags=re.M | re.S)
    text = re.sub(r'(\s*\n)+', '\n', text, flags=re.M | re.S)
    return unescape(text)
