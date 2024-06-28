<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create assignment prelist by user &amp; group</name>
   <tag></tag>
   <elementGuidId>7e577b8d-99fe-4ea1-85b3-c8133a65c85e</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;data\&quot;: [\n    {\n      \&quot;group\&quot;: \&quot;663c9117ec85df3cf9808af6\&quot;,\n      \&quot;user\&quot;: \&quot;663dcf5110f4f14579798e4e\&quot;\n    }\n  ]\n}&quot;,
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
      <webElementGuid>5851b5ea-2e99-47c0-af73-2ebf4e10974a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>0a3dbd7e-1031-4c84-95d3-7f499e58e278</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${assUrl}${pr}unassign/by-user</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ASSIGNMENT_BASE_URL</defaultValue>
      <description></description>
      <id>a1e7687e-2648-42e9-96dc-79ebde4f1303</id>
      <masked>false</masked>
      <name>assUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.prelistEndpoint</defaultValue>
      <description></description>
      <id>4adb4ae6-cba4-4529-b25e-4f1fcf3c3eaf</id>
      <masked>false</masked>
      <name>pr</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>0bee4705-e14b-4c0a-b9e3-5a4499c0642c</id>
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
