<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>bulk update reader group or status</name>
   <tag></tag>
   <elementGuidId>cbd73f1a-6117-4bb4-8b6b-831762c304b7</elementGuidId>
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
  &quot;text&quot;: &quot;[\n  {\n    \&quot;id\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;group\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;status\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;networkStatus\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;scanStatus\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;startDate\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;power\&quot;: \&quot;\u003cnumber\u003e\&quot;\n  },\n  {\n    \&quot;id\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;group\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;status\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;networkStatus\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;scanStatus\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;startDate\&quot;: \&quot;\u003cstring\u003e\&quot;,\n    \&quot;power\&quot;: \&quot;\u003cnumber\u003e\&quot;\n  }\n]&quot;,
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
      <webElementGuid>5bfe5430-8bcf-4d36-ac7d-22402c6f8e63</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ba2bf1ab-8a2a-4000-990e-3f6f11f72141</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>2419890e-68c3-4c93-8dbe-03c815328906</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${devUrl}${iotRed}bulk-update</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.DEVICES_BASE_URL</defaultValue>
      <description></description>
      <id>55599137-03dc-4e6f-bf57-5cf5b741d3cf</id>
      <masked>false</masked>
      <name>devUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.iotReader</defaultValue>
      <description></description>
      <id>15f2a757-f782-4be9-94ca-6e3dea047647</id>
      <masked>false</masked>
      <name>iotRed</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>05cfd166-4b54-4b27-910f-e16533fae5a5</id>
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
