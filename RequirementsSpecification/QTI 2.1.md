<?xml version="1.0" encoding="UTF-8"?>
<assessmentItem
    xmlns="http://www.imsglobal.org/xsd/imsqti_v2p1"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.imsglobal.org/xsd/imsqti_v2p1 
    http://www.imsglobal.org/xsd/qti/qtiv2p1/imsqti_v2p1.xsd"
    identifier="item_001"
    title="単一解答の多肢選択問題"
    adaptive="false"
    timeDependent="false">

  <responseDeclaration identifier="RESPONSE" cardinality="single" baseType="identifier">
    <correctResponse>
      <value>choice_3</value>
    </correctResponse>
  </responseDeclaration>

  <outcomeDeclaration identifier="SCORE" cardinality="single" baseType="float">
    <defaultValue>
      <value>0</value>
    </defaultValue>
  </outcomeDeclaration>

  <itemBody>
    <choiceInteraction responseIdentifier="RESPONSE" shuffle="false" maxChoices="1">
      <prompt>日本の首都はどこですか。</prompt>

      <simpleChoice identifier="choice_1">大阪</simpleChoice>
      <simpleChoice identifier="choice_2">京都</simpleChoice>
      <simpleChoice identifier="choice_3">東京</simpleChoice>
      <simpleChoice identifier="choice_4">名古屋</simpleChoice>
    </choiceInteraction>
  </itemBody>

  <responseProcessing
      template="http://www.imsglobal.org/question/qti_v2p1/rptemplates/match_correct"/>
</assessmentItem>