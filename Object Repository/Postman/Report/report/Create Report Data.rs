<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Report Data</name>
   <tag></tag>
   <elementGuidId>bca5c0a6-edc0-4997-aab7-4283700f5888</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${bt}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;asdf report\&quot;,\n  \&quot;type\&quot;: \&quot;Custom Report\&quot;,\n  \&quot;assetInformation\&quot;: \&quot;Basic Information\&quot;,\n  \&quot;listedBy\&quot;: \&quot;Group\&quot;,\n  \&quot;startDate\&quot;: \&quot;2024-07-02\&quot;,\n  \&quot;endDate\&quot;: \&quot;2024-07-02\&quot;,\n  \&quot;startfrom\&quot;: \&quot;2024-07-02\&quot;,\n  \&quot;tableColumn\&quot;: [\n    \&quot;Asset ID\&quot;\n  ],\n  \&quot;frequency\&quot;: {\n    \&quot;label\&quot;: \&quot;Daily\&quot;,\n    \&quot;customUnit\&quot;: \&quot;Year(s)\&quot;,\n    \&quot;customValue\&quot;: 1\n  },\n  \&quot;email\&quot;: {\n    \&quot;recipient\&quot;: [\n      \&quot;a@mail.com\&quot;\n    ],\n    \&quot;type\&quot;: [\n      \&quot;a@mail.com\&quot;\n    ],\n    \&quot;cc\&quot;: [\n      \&quot;a@mail.com\&quot;\n    ],\n    \&quot;subject\&quot;: \&quot;subject\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>fefce9b0-7277-48ae-8d82-a182e533f289</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>c6ac2b37-acf8-47ad-b66d-5afc04056106</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>5ea0b4e2-af3a-49a8-8101-bbe45d7dd9ee</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${repUrl}${rep}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.REPORT_BASE_URL</defaultValue>
      <description></description>
      <id>3a475165-96c7-4ec6-9382-5bc0374ebf97</id>
      <masked>false</masked>
      <name>repUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reportEndpoint</defaultValue>
      <description></description>
      <id>e7a77cd1-93b6-415a-a397-21b3bf02c9a1</id>
      <masked>false</masked>
      <name>rep</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>f3734065-9df2-47da-8b27-bf9abea2d07e</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
