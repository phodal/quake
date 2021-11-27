import sqlite3
import zlib


def GetUncompressedData(compressed):
    if compressed == None:
        return None
    data = None
    try:
        wbits = 15 + 32
        data = zlib.decompress(compressed, wbits)
    except zlib.error:
        raise print('Zlib Decompression failed!')
    return data


db = sqlite3.connect("../../NoteStore.sqlite")
db.row_factory = sqlite3.Row

query = " SELECT n.Z_PK, n.ZNOTE as note_id, n.ZDATA as data, " \
        " c3.ZFILESIZE, " \
        " c4.ZFILENAME, c4.ZIDENTIFIER as att_uuid,  " \
        " c1.ZTITLE1 as title, c1.ZSNIPPET as snippet, c1.ZIDENTIFIER as noteID, " \
        " c1.ZCREATIONDATE1 as created, c1.ZLASTVIEWEDMODIFICATIONDATE, c1.ZMODIFICATIONDATE1 as modified, " \
        " c2.ZACCOUNT3, c2.ZTITLE2 as folderName, c2.ZIDENTIFIER as folderID, " \
        " c5.ZNAME as acc_name, c5.ZIDENTIFIER as acc_identifier, c5.ZACCOUNTTYPE " \
        " FROM ZICNOTEDATA as n " \
        " LEFT JOIN ZICCLOUDSYNCINGOBJECT as c1 ON c1.ZNOTEDATA = n.Z_PK  " \
        " LEFT JOIN ZICCLOUDSYNCINGOBJECT as c2 ON c2.Z_PK = c1.ZFOLDER " \
        " LEFT JOIN ZICCLOUDSYNCINGOBJECT as c3 ON c3.ZNOTE= n.ZNOTE " \
        " LEFT JOIN ZICCLOUDSYNCINGOBJECT as c4 ON c4.ZATTACHMENT1= c3.Z_PK " \
        " LEFT JOIN ZICCLOUDSYNCINGOBJECT as c5 ON c5.Z_PK = c1.ZACCOUNT2  " \
        " ORDER BY note_id  "

db.row_factory = sqlite3.Row
cursor = db.execute(query)
for row in cursor:
    try:
        data = GetUncompressedData(row['data'])
        print(data)
    except sqlite3.Error:
        raise print('error')

