<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Routine Schedule</name>
   <tag></tag>
   <elementGuidId>87e54e3d-7407-4152-a8fe-5b7581616418</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;frequency\&quot;: {\n    \&quot;label\&quot;: \&quot;\u003cstring\u003e\&quot;\n  },\n  \&quot;maintenanceIndicator\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;assetNames\&quot;: [\n    {\n      \&quot;_id\&quot;: \&quot;\u003cstring\u003e\&quot;,\n      \&quot;group\&quot;: \&quot;\u003cstring\u003e\&quot;\n    },\n    {\n      \&quot;_id\&quot;: \&quot;\u003cstring\u003e\&quot;,\n      \&quot;group\&quot;: \&quot;\u003cstring\u003e\&quot;\n    }\n  ],\n  \&quot;staff\&quot;: [\n    \&quot;\u003cstring\u003e\&quot;,\n    \&quot;\u003cstring\u003e\&quot;\n  ],\n  \&quot;taskName\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;nextSchedule\&quot;: \&quot;\u003cdate\u003e\&quot;,\n  \&quot;duration\&quot;: \&quot;\u003cnumber\u003e\&quot;\n}&quot;,
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
      <webElementGuid>68589399-4b1b-4d89-9afa-db8e9fa55e48</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>80ce2350-5862-4bf2-b4d2-bfd17c62adec</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${rutUrl}${rutSch}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ROUTINE_BASE_URL</defaultValue>
      <description></description>
      <id>5bf8edb0-83a9-46e6-9d18-28edeea80879</id>
      <masked>false</masked>
      <name>rutUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.routineSchedule</defaultValue>
      <description></description>
      <id>c78601ac-95a3-45bb-85d9-0d3b7ed74194</id>
      <masked>false</masked>
      <name>rutSch</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>b7a431aa-819f-4400-b64f-7a7a6524b05c</id>
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
