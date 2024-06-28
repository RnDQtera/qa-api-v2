<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create disposal prelist</name>
   <tag></tag>
   <elementGuidId>3701c46d-6321-4fd4-b44c-1a2fc212b2a6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY0NTRjNDI1M2RkMTAxN2YyODY4OTU3MiIsImNvbXBhbnkiOiI2NWVhNzhhYmY0NTgyNTc4NWI2OGEyZDciLCJjb21wYW55Q29kZSI6IjAwMHYyIiwic2Vzc2lvbiI6IjY2N2U2YzU3ZGVhZDI0ZGMxMWU1ZWZlNiIsImlhdCI6MTcxOTU2MTMwMywiZXhwIjoxNzE5NjA0NTAzfQ.2FVgKTVvAshWwiB_bpetxGz3hHyV5kd1Wrn2Y_RSsBI</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: [\n    \&quot;661f906e6b1266ce094a4de9\&quot;\n  ],\n  \&quot;disposalGroup\&quot;: \&quot;645e40976274c7513437248a\&quot;,\n  \&quot;organization\&quot;: \&quot;PT.ASDF\&quot;,\n  \&quot;method\&quot;: \&quot;Sell\&quot;,\n  \&quot;reason\&quot;: \&quot;Damaged\&quot;,\n  \&quot;notes\&quot;: \&quot;Damaged\&quot;\n}&quot;,
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
      <webElementGuid>5b723613-8439-4627-97db-c0230cd9aadc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>251af1b1-af2f-419a-bdc2-3e77897b7ad3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY0NTRjNDI1M2RkMTAxN2YyODY4OTU3MiIsImNvbXBhbnkiOiI2NWVhNzhhYmY0NTgyNTc4NWI2OGEyZDciLCJjb21wYW55Q29kZSI6IjAwMHYyIiwic2Vzc2lvbiI6IjY2N2U2YzU3ZGVhZDI0ZGMxMWU1ZWZlNiIsImlhdCI6MTcxOTU2MTMwMywiZXhwIjoxNzE5NjA0NTAzfQ.2FVgKTVvAshWwiB_bpetxGz3hHyV5kd1Wrn2Y_RSsBI</value>
      <webElementGuid>029f3116-fd39-4faa-918e-32e5c721988e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${disUrl}${disPr}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.DISPOSAL_BASE_URL</defaultValue>
      <description></description>
      <id>dd023946-ab5e-4547-9987-7b5f24c05527</id>
      <masked>false</masked>
      <name>disUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.disposalPrelist</defaultValue>
      <description></description>
      <id>0551c0a0-3538-4cce-a936-ada5730704f5</id>
      <masked>false</masked>
      <name>disPr</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Options</defaultValue>
      <description></description>
      <id>a59c2ba7-668d-4d1c-a24e-ba867f1fe291</id>
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
