import re
import markdown
import math
import lxml
from pyquery import PyQuery as pq


WORD_DELIMITER = re.compile(r"\W+")
WPM = 265


"""
Credit: Alan Hamlett -- readtime library (https://github.com/alanhamlett/readtime)
"""


def read_time(content, format="markdown", wpm=WPM):
    """Returns the read time of some content.
    :param content: String of content.
    :param format: Format of the content (html, markdown, or text).
    """

    try:
        format = format.lower()
    except:
        pass

    if format == "text":
        seconds = read_time_as_seconds(content, wpm=wpm)

    elif format == "markdown":
        html = markdown.markdown(content)
        el = pq(html)
        text, images = parse_html(el)
        seconds = read_time_as_seconds(text, images=images, wpm=wpm)

    return max(1, math.ceil(seconds / 60))


def read_time_as_seconds(text, images=0, wpm=WPM):
    """Returns the read time as seconds of some plain text.
    :param text:   String of plain text.
    :param images: The number of inline images in the text.
    """

    try:
        num_words = len(re.split(WORD_DELIMITER, text.strip()))
    except (AttributeError, TypeError):
        num_words = 0

    seconds = math.ceil(num_words / wpm * 60)

    # add extra seconds for inline images
    delta = 12
    for _ in range(images):
        seconds += delta
        if delta > 3:
            delta -= 1

    return seconds


def parse_html(el):
    """Converts HTML to plain text.
    Returns a tuple of (plain_text, num_images).
    :param el: A PyQuery DOM object.
    """

    text = []
    images = []
    headers = ["h1", "h2", "h3", "h4", "h5"]

    def add_text(tag, no_tail=False):
        if tag.tag == "img":
            images.append(tag)
        if tag.text and not isinstance(tag, lxml.etree._Comment):
            text.append(tag.text)
        for child in tag.getchildren():
            add_text(child)
        if tag.tag in headers and len(text) > 0 and not text[-1].strip().endswith("."):
            text.append(".")
        if not no_tail and tag.tail:
            text.append(tag.tail)

    for tag in el:
        add_text(tag, no_tail=True)

    plain_text = re.sub(r"\s+", " ", "".join([t for t in text if t])).strip()

    return plain_text, len(images)
