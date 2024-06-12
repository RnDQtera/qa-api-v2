<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Service Center </name>
   <tag></tag>
   <elementGuidId>cc54a6d1-db99-4213-9029-1587c63751f2</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;state\&quot;: {\n    \&quot;name\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;iso\&quot;: \&quot;\u003cstring\u003e\&quot;\n  },\n  \&quot;country\&quot;: {\n    \&quot;name\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;iso\&quot;: \&quot;\u003cstring\u003e\&quot;\n  },\n  \&quot;name\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;category\&quot;: \&quot;\u003carray\u003e\&quot;,\n  \&quot;brand\&quot;: \&quot;\u003carray\u003e\&quot;,\n  \&quot;zipCode\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;phoneNumber\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;picName\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;address\&quot;: \&quot;\u003cstring\u003e\&quot;,\n  \&quot;picPhoneNumber\&quot;: \&quot;\u003cstring\u003e\&quot;\n}&quot;,
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
      <webElementGuid>ddaf621d-7ab3-4c27-8d31-ccccf1bbeb82</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b13fe27d-d14f-4c8f-b87e-a7f35d3ec2ff</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>8d9c79ea-c4a5-4dd0-b3be-db7efd22a0dd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${rutUrl}${serCen}:id</restUrl>
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
      <id>fa60b14a-4aa2-4ebf-a170-c52a7850458f</id>
      <masked>false</masked>
      <name>rutUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.serviceCenter</defaultValue>
      <description></description>
      <id>3c1c6806-7901-40b9-8b82-b2911d9a45dd</id>
      <masked>false</masked>
      <name>serCen</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>ac487f6f-f15f-4fff-96e5-60ae6a16e754</id>
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
