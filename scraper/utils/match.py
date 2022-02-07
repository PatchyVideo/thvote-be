import re


async def match_bilibili(text: str) -> bool:
    if re.match(r'.*((?:http|https)://(?:(?:bili(?:22|23|33|2233).cn)|(?:b23.tv))/[A-Za-z0-9]+)', text, re.DOTALL):
        # 短链接
        return True
    if re.match(r'.*(?<![A-Za-z0-9])(?:AV|av)(\d+)', text, re.DOTALL):
        # av号
        return True
    if re.match(r'.*(?<![A-Za-z0-9])(BV[A-Za-z0-9]{10})(?![A-Za-z0-9])', text, re.DOTALL):
        # bv号
        return True
    return False


async def match_twitter(text: str) -> bool:
    if re.match(r'.*(https:\/\/)?(www\.|mobile\.)?twitter\.com\/[\w]+\/status\/[\d]+', text):
        return True
    return False
