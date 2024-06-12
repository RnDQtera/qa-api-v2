<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update audit schedule details</name>
   <tag></tag>
   <elementGuidId>b3d98281-101a-4e73-bdda-6f0471bb9856</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;customValue\&quot;: {\n    \&quot;value\&quot;: \&quot;\u003cnumber\u003e\&quot;,\n    \&quot;frequency\&quot;: \&quot;\u003cstring\u003e\&quot;\n  },\n  \&quot;taskName\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;category\&quot;: \&quot;\u003carray\u003e\&quot;,\n  \&quot;auditType\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;auditDate\&quot;: \&quot;\u003cdate\u003e\&quot;,\n  \&quot;frequency\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;duration\&quot;: \&quot;\u003cnumber\u003e\&quot;\n}&quot;,
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
      <webElementGuid>7786497d-3c16-4f4b-acf6-e7e53e2ff01d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>1f8c28d1-fe36-41ae-8325-439538dac36a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>7b6c0eb8-2ac3-4ed8-8612-fd04fce4759a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${auUrl}${auSch}/:id</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.AUDIT_BASE_URL</defaultValue>
      <description></description>
      <id>749f0c80-e44b-4abd-a132-27f75a0ddd95</id>
      <masked>false</masked>
      <name>auUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.auditSchedule</defaultValue>
      <description></description>
      <id>3e6b7fdb-1aea-4627-abb1-9ad6b6a2d77d</id>
      <masked>false</masked>
      <name>auSch</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>3cfa7b51-bc4a-45df-94e4-fd546cab5dd4</id>
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
