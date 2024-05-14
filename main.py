import re

#conversation = input("Conversation to be analyse:\n\n")
source_file = open("WhatsApp Chat with May\WhatsApp Chat with May.txt", "r", encoding="utf8")

conversation = source_file.read()
conversation = re.sub(r".*:", "", conversation, flags=re.MULTILINE)


conversation = conversation.replace(".", " ").replace(",", " ").replace("!", " ").replace("?", " ")
conversation = conversation.replace('\n', " ").replace("\t", " ").replace('\r', " ")
conversation = conversation.replace("<Media omitted>", " ")

words = conversation.lower().split(" ")


word_counter = {}

for word in words:
    if word in word_counter:
        word_counter[word] += 1
    else:
        word_counter[word] = 1


i = 0;
for item in sorted(word_counter.items(), key = lambda x: x[1], reverse = True):
    if i < 100:
        print(item)
    else:
        break

    i += 1

print("\n-----\nCOMPLETED")