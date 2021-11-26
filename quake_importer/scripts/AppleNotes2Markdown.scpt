-- base on : https://github.com/mattrose/AppleNotes2Joplin
-- Prompt user to choose folder for export files and then create it in filesystem

set exportfolder to choose folder with prompt "Please choose a folder to export your Apple notes to:" default location (path to home folder from user domain)
set outfolder to POSIX path of exportfolder as string
do shell script "mkdir -p " & quoted form of (POSIX path of outfolder)

-- Let the user select which folders from Apple Notes to export

tell application "Notes"
	set folderNames to name of folders
	set chosenFolderNames to (choose from list folderNames with multiple selections allowed)
	if (chosenFolderNames is false) then error number -128 -- Cancel button
end tell

repeat with i from 1 to (count chosenFolderNames)
	
	-- For each folder selected to export, start setting up corresponding Joplin export files including an md file for the folder Joplin parent_id
	
	set parent_id to do shell script "uuidgen | tr -d '-' | tr 'A-Z' 'a-z' "
	
	set thisFolderName to item i of chosenFolderNames
	tell application "Notes" to set theNotes to notes of folder thisFolderName
	
	do shell script "mkdir -p " & quoted form of (POSIX path of outfolder) & "/" & "resources/"
	
	tell application "Notes"
		set localdirDate to the modification date of note 1 of folder thisFolderName
	end tell
	
	set UTCdirdate to localdirDate - (time to GMT)
	set dyy to year of UTCdirdate as integer
	set dMM to month of UTCdirdate as integer
	set dDD to day of UTCdirdate as integer
	set dHH to hours of UTCdirdate as integer
	set dmins to minutes of UTCdirdate as integer
	set dsecs to seconds of UTCdirdate as integer
	set trzdMM to text -2 thru -1 of ("0" & dMM)
	set trzdDD to text -2 thru -1 of ("0" & dDD)
	set trzdHH to text -2 thru -1 of ("0" & dHH)
	set trzdmins to text -2 thru -1 of ("0" & dmins)
	set trzdsecs to text -2 thru -1 of ("0" & dsecs)
	
	set dirText to thisFolderName & "

id: " & parent_id & "
created_date: " & dyy & "-" & trzdMM & "-" & trzdDD & "T" & trzdHH & ":" & trzdmins & ":" & trzdsecs & ".000" & "Z" & "
updated_date: " & dyy & "-" & trzdMM & "-" & trzdDD & "T" & trzdHH & ":" & trzdmins & ":" & trzdsecs & ".000" & "Z" & "
user_created_date: " & dyy & "-" & trzdMM & "-" & trzdDD & "T" & trzdHH & ":" & trzdmins & ":" & trzdsecs & ".000" & "Z" & "
user_updated_date: " & dyy & "-" & trzdMM & "-" & trzdDD & "T" & trzdHH & ":" & trzdmins & ":" & trzdsecs & ".000" & "Z" & "
parent_id:
"
	
	do shell script "printf  " & quoted form of dirText & " >  " & quoted form of (POSIX path of outfolder & parent_id & ".md")
	do shell script "perl -pi -e 's/\\r\\n|\\n|\\r/\\n/g'  " & quoted form of (POSIX path of outfolder & parent_id & ".md")
	
	-- Export all Apple notes in this folder
	
	repeat with j from 1 to (count theNotes)
		
		-- Open the folder in Notes.app interface
		
		tell application "Notes"
			tell account "iCloud"
				tell folder thisFolderName
					show note j
				end tell
			end tell
		end tell
		
		-- Click mouse in top note to make sure the (rich) text of the note is successfully copied to the clipboard via this script's method
		
		tell application "System Events"
			tell process "Notes"
				try
					set focused of text area 1 of scroll area 1 of group 1 of splitter group 1 of splitter group 1 of window "Notes" to true
				end try
			end tell
		end tell
		
		tell application "System Events"
			tell process "Notes"
				click menu item "Select All" of menu "Edit" of menu bar 1
				click menu item "Copy" of menu "Edit" of menu bar 1
			end tell
		end tell
		
		delay 0.5 -- A little delay to help avoid hiccups
		
		do shell script "osascript -e 'the clipboard as �class RTF �' | perl -ne 'print chr foreach unpack(\"C*\",pack(\"H*\",substr($_,11,-3)))' | textutil -stdin -stdout -convert html -format rtf | pbcopy"
		
		-- Raw HTML tool: Uncomment the section below to grab the original raw HTML of the current Apple note body. May help to identify what basic HTML formatting code would work for the note in Joplin, or potentially making this script better if someone wants to help with that, see README.
		
		(* tell application "Notes"
			tell account "iCloud"
				tell folder thisFolderName
					set rawbodyhtml to get the body of note j
					display dialog rawbodyhtml
				end tell
			end tell
		end tell *)
		
		delay 0.5 -- A little delay to help avoid hiccups
		
		-- Translate this Apple note's data into the Joplin format
		
		set joplinid to do shell script "uuidgen | tr -d '-' | tr 'A-Z' 'a-z' "
		
		tell application "Notes"
			tell account "iCloud"
				tell folder thisFolderName
					set localcreateDate to get the creation date of note j
				end tell
			end tell
		end tell
		
		set UTCcreatedate to localcreateDate - (time to GMT)
		
		set cyy to year of UTCcreatedate as integer
		set cMM to month of UTCcreatedate as integer
		set cDD to day of UTCcreatedate as integer
		set cHH to hours of UTCcreatedate as integer
		set cmins to minutes of UTCcreatedate as integer
		set csecs to seconds of UTCcreatedate as integer
		set trzcMM to text -2 thru -1 of ("0" & cMM)
		set trzcDD to text -2 thru -1 of ("0" & cDD)
		set trzcHH to text -2 thru -1 of ("0" & cHH)
		set trzcmins to text -2 thru -1 of ("0" & cmins)
		set trzcsecs to text -2 thru -1 of ("0" & csecs)
		
		tell application "Notes"
			tell account "iCloud"
				tell folder thisFolderName
					set localeditDate to the modification date of note j
				end tell
			end tell
		end tell
		
		set UTCeditdate to localeditDate - (time to GMT)
		set eyy to year of UTCeditdate as integer
		set eMM to month of UTCeditdate as integer
		set eDD to day of UTCeditdate as integer
		set eHH to hours of UTCeditdate as integer
		set emins to minutes of UTCeditdate as integer
		set esecs to seconds of UTCeditdate as integer
		set trzeMM to text -2 thru -1 of ("0" & eMM)
		set trzeDD to text -2 thru -1 of ("0" & eDD)
		set trzeHH to text -2 thru -1 of ("0" & eHH)
		set trzemins to text -2 thru -1 of ("0" & emins)
		set trzesecs to text -2 thru -1 of ("0" & esecs)
		
		set myText to ((do shell script "pbpaste")) & ""
		
		set myText to myText & "id: " & joplinid & "
parent_id: " & parent_id & "
created_date: " & cyy & "-" & trzcMM & "-" & trzcDD & "T" & trzcHH & ":" & trzcmins & ":" & trzcsecs & ".000" & "Z" & "
updated_date: " & eyy & "-" & trzeMM & "-" & trzeDD & "T" & trzeHH & ":" & trzemins & ":" & trzesecs & ".000" & "Z" & "
first_name: Phodal
last_name: HUANG
email: h@phdoal.com
user_created_time: " & cyy & "-" & trzcMM & "-" & trzcDD & "T" & trzcHH & ":" & trzcmins & ":" & trzcsecs & ".000" & "Z" & "
user_updated_time: " & eyy & "-" & trzeMM & "-" & trzeDD & "T" & trzeHH & ":" & trzemins & ":" & trzesecs & ".000" & "Z" & "
"
		
		-- Clean up Apple note's HTML code and do some basic HTML to MD conversion
		
		set deleteHTMLline to {"<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01//EN\" \"http://www.w3.org/TR/html4/strict.dtd\">"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<html>"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<head>"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\">"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<meta http-equiv=\"Content-Style-Type\" content=\"text/css\">"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<title></title>"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<meta name=\"Generator\" content=\"Cocoa HTML Writer\">"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<meta name=\"CocoaVersion\" content=\"2113\">"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p1 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p2 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p3 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p4 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p5 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p6 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p7 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p8 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p9 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p10 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p11 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"li.li1 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"li.li2 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"li.li3 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"li.li4 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"li.li5 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"li.li6 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"li.li7 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"li.li8 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"</style>"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"</head>"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<body>"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<style type=\"text/css\">"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p4 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"p.p5 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"span.s1 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"span.s2 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"span.s3 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"span.s4 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"span.s5 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"span.s6 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"span.s7 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"span.Apple-tab-span {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"table.t1 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"td.td1 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"ul.ul1 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"ul.ul2 {"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<ul class=\"ul1\">"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<ul class=\"ul2\">"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"</ul>"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"ol.ol1 {list-style-type: decimal}"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"<ol class=\"ol1\">"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		set deleteHTMLline to {"</ol>"}
		set myText to deleteLinesFromText(myText, deleteHTMLline) of me as text
		
		set myText to replace_chars(myText, "<!DOCTYPE*<body>", "") of me as text
		set myText to replace_chars(myText, "</b>", "**") of me as text
		set myText to replace_chars(myText, "</body>", "") of me as text
		set myText to replace_chars(myText, "</div>", "") of me as text
		set myText to replace_chars(myText, "</html>", "") of me as text
		set myText to replace_chars(myText, "</i>", "*") of me as text
		set myText to replace_chars(myText, "</li>", "") of me as text
		set myText to replace_chars(myText, "</p>", "") of me as text
		set myText to replace_chars(myText, "</span>", "") of me as text
		set myText to replace_chars(myText, "<b>", "**") of me as text
		set myText to replace_chars(myText, "<br>", "") of me as text
		set myText to replace_chars(myText, "<div>", "") of me as text
		set myText to replace_chars(myText, "<i>", "*") of me as text
		set myText to replace_chars(myText, "<li class=\"li1\">", "<li>") of me as text
		set myText to replace_chars(myText, "<li class=\"li2\">", "<li>") of me as text
		set myText to replace_chars(myText, "<li class=\"li3\">", "<li>") of me as text
		set myText to replace_chars(myText, "<li class=\"li4\">", "<li>") of me as text
		set myText to replace_chars(myText, "<li class=\"li5\">", "<li>") of me as text
		set myText to replace_chars(myText, "<li class=\"li6\">", "<li>") of me as text
		set myText to replace_chars(myText, "<li class=\"li7\">", "<li>") of me as text
		set myText to replace_chars(myText, "<li class=\"li8\">", "<li>") of me as text
		set myText to replace_chars(myText, "<p class=\"p1\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p2\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p3\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p4\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p5\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p6\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p7\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p8\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p9\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p10\">", "") of me as text
		set myText to replace_chars(myText, "<p class=\"p11\">", "") of me as text
		set myText to replace_chars(myText, "<span class=\"Apple-converted-space\">", "") of me as text
		set myText to replace_chars(myText, "<span class=\"Apple-tab-span\">", "") of me as text
		set myText to replace_chars(myText, "<span class=\"s1\">", "") of me as text
		set myText to replace_chars(myText, "<span class=\"s2\">", "") of me as text
		set myText to replace_chars(myText, "<span class=\"s3\">", "") of me as text
		set myText to replace_chars(myText, "<span class=\"s4\">", "") of me as text
		set myText to replace_chars(myText, "<span class=\"s5\">", "") of me as text
		set myText to replace_chars(myText, "<span class=\"s6\">", "") of me as text
		set myText to replace_chars(myText, "<span class=\"s7\">", "") of me as text
		set myText to replace_chars(myText, "<ul class=\"ul1\">", "<ul>") of me as text
		
		-- Convert the indentation element of bulleted lists from HTML to Markdown. This code should be improved. It's finite only for 10 levels without being able to do any 'n' number of indents.
		
		set myText to replace_chars(myText, "                    <li>", tab & tab & tab & tab & tab & tab & tab & tab & tab & "- ") of me as text
		set myText to replace_chars(myText, "                  <li>", tab & tab & tab & tab & tab & tab & tab & tab & "- ") of me as text
		set myText to replace_chars(myText, "                <li>", tab & tab & tab & tab & tab & tab & tab & "- ") of me as text
		set myText to replace_chars(myText, "              <li>", tab & tab & tab & tab & tab & tab & "- ") of me as text
		set myText to replace_chars(myText, "            <li>", tab & tab & tab & tab & tab & "- ") of me as text
		set myText to replace_chars(myText, "          <li>", tab & tab & tab & tab & "- ") of me as text
		set myText to replace_chars(myText, "        <li>", tab & tab & tab & "- ") of me as text
		set myText to replace_chars(myText, "      <li>", tab & tab & "- ") of me as text
		set myText to replace_chars(myText, "    <li>", tab & "- ") of me as text
		set myText to replace_chars(myText, "  <li>", "- ") of me as text
		
		-- Convert HTML hyperlinks to Markdown format
		
		set myText2 to do shell script "perl -pe 's/<a.*?href=\"(.*?)\".*?>(.*?)<\\/a>/[\\2](\\1)/g' <<< " & myText's quoted form
		
		-- Now that the Apple note's underlying HTML has been cleaned up or converted to Markdown, unescape certain characters from the Apple note's user text that were escaped during earlier processing, for convenience in Joplin's code editor.
		
		set myText2 to replace_chars(myText2, "&amp;", "&") of me as text
		set myText2 to replace_chars(myText2, "&lt;", "<") of me as text
		set myText2 to replace_chars(myText2, "&gt;", ">") of me as text
		
		-- Clean up other anomolies
		
		set myText2 to replace_chars(myText2, return & "****" & return, return & "" & return) of me as text -- This cleans up empty lines of bold text in an Apple note. If you specifically use lines in your text only comprising "****", delete this line.
		set myText2 to replace_chars(myText2, return & "****" & return, return & "" & return) of me as text -- Second pass of the above needed for some instances
		
		set myText2 to replace_chars(myText2, return & "**" & return, return & "" & return) of me as text -- This cleans up empty lines of italics text in an Apple note. If you specifically use lines in your text only comprising "**", delete this line.
		set myText2 to replace_chars(myText2, return & "**" & return, return & "" & return) of me as text -- Second pass of the above needed  for some instances
		
		-- Make a duplication of the first line of note (whether it had a title or not) with paragraph spacing to make the note's first line od body correctly be such in Joplin. If you use specific titles in all your Apple notes, it will look cleaner in Joplin if you delete this bit.
		
		set myText2 to item 1 of paragraphs of myText2 & "

" & myText2
		
		-- Write out the note file to its Joplin-ready RAW .md
		
		do shell script "echo  " & quoted form of myText2 & " >  " & quoted form of (POSIX path of outfolder & joplinid & ".md")
		
		-- Convert CR to LF in the file text
		
		do shell script "perl -pi -e 's/\\r\\n|\\n|\\r/\\n/g'  " & quoted form of (POSIX path of outfolder & joplinid & ".md")
		
		-- Remove the empty line at end of file
		
		do shell script "perl -i -pe 'chomp if eof'  " & quoted form of (POSIX path of outfolder & joplinid & ".md")
		
		delay 0.5 -- A little delay to help avoid hiccups
		
	end repeat
end repeat

-- The AppleScript code to do text replacement

on replace_chars(this_text, search_string, replacement_string)
	set AppleScript's text item delimiters to the search_string
	set the item_list to every text item of this_text
	set AppleScript's text item delimiters to the replacement_string
	set this_text to the item_list as string
	set AppleScript's text item delimiters to {""}
	return this_text
end replace_chars

-- The AppleScript code to do deletion of lines containing X

on deleteLinesFromText(theText, deletePhrase)
	set newText to ""
	try
		set textList to paragraphs of theText
		repeat with i from 1 to count of textList
			set thisLine to item i of textList
			if thisLine does not contain deletePhrase then
				set newText to newText & thisLine & return
			end if
		end repeat
		if newText is not "" then set newText to text 1 thru -2 of newText
	on error
		set newText to theText
	end try
	return newText
end deleteLinesFromText

-- Joplin note format specs for handy reference: https://joplinapp.org/api/references/rest_api/
