<?xml version="1.0" encoding="UTF-8"?>
<assessmentItem
    xmlns="http://www.imsglobal.org/xsd/imsqti_v2p2"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.imsglobal.org/xsd/imsqti_v2p2 https://purl.imsglobal.org/spec/qti/v2p2/schema/xsd/imsqti_v2p2p4.xsd"
    identifier="item001"
    title="単一解答の多肢選択問題"
    adaptive="false"
    timeDependent="false">

    <responseDeclaration identifier="RESPONSE" cardinality="single" baseType="identifier">
        <correctResponse>
            <value>choice_2</value>
        </correctResponse>
    </responseDeclaration>
    
    <outcomeDeclaration identifier="SCORE" cardinality="single" baseType="float">
        <defaultValue>
            <value>0</value>
        </defaultValue>
    </outcomeDeclaration>
    
    <itemBody>
        <p>日本の首都はどこですか。</p>
    
        <choiceInteraction responseIdentifier="RESPONSE" shuffle="false" maxChoices="1">
            <prompt>正しいものを1つ選びなさい。</prompt>
            <simpleChoice identifier="choice_1">大阪</simpleChoice>
            <simpleChoice identifier="choice_2">東京</simpleChoice>
            <simpleChoice identifier="choice_3">京都</simpleChoice>
            <simpleChoice identifier="choice_4">名古屋</simpleChoice>
        </choiceInteraction>
    </itemBody>
    
    <responseProcessing template="https://www.imsglobal.org/question/qti_v2p2/rptemplates/match_correct"/>
</assessmentItem>