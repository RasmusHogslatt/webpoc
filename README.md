# Magazine structure

**Magazine**:
A **Magazine** contains a list of slots containing **Content**. Each **Content** can be seen as one row that will be shown in the web application.

**Content**:
1. Tool number: A text such as TG101 manually written.
2. Tool: Described below.
3. Holder: Described below.
4. Overhang extension: A value
5. A description in text format. Hoverering should show all text if text is too long.

Rasmus notes: Looking at the "räknesticka" file on the page "Ställ & VTG", there are more fields such as diameter and length which gives me the questions...

Question 1: Should these be part of the **Tool** or added manually into **Content**? 

Question 2: Do you want to be able to sort by these values?

Dad's thoughts:

**Tool**
What types of tools should be available? 

Option 1: Categorize tools into *Rotating*, *Turning*, *etc*, and then further divide into subcategories? This would cause many user options and button clicks.

Option 2: Directly choose the exact type of tool. For example *Drill* or *Trigon Insert*. When coding, I could still make it possible to get the category of any tool if I know, at the time of coding, if a *Drill* is a *Rotating* or *Turning* tool for example. I would also need to know what parameters you want to add manually, like *manufacturer* etc. If so, It would be nice to have list of tools containing:

Tool: Drill, Category: Rotating
Tool: Trigon Insert: Category: Turning

Note: Are there more categories of tools you want support for? If so, could they be added later?

**Holder**
What types of holders should be available
