<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Service Center  </name>
   <tag></tag>
   <elementGuidId>2791d0c3-e859-4979-8417-b9b53284fa9b</elementGuidId>
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
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;Jendela Service Center\&quot;,\n  \&quot;category\&quot;: [\n    13\n  ],\n  \&quot;brand\&quot;: [\n    131\n  ],\n  \&quot;state\&quot;: {\n    \&quot;name\&quot;: \&quot;Jkt\&quot;,\n    \&quot;iso\&quot;: \&quot;Jkt-iso\&quot;\n  },\n  \&quot;country\&quot;: {\n    \&quot;name\&quot;: \&quot;Indonesia\&quot;,\n    \&quot;iso\&quot;: \&quot;Indonesia-iso\&quot;\n  },\n  \&quot;city\&quot;: \&quot;Jakut\&quot;,\n  \&quot;zipCode\&quot;: \&quot;aesdeep\&quot;,\n  \&quot;phoneNumber\&quot;: \&quot;505050\&quot;,\n  \&quot;picName\&quot;: \&quot;Agam\&quot;,\n  \&quot;address\&quot;: \&quot;Rukan Villa Gading Indah\&quot;,\n  \&quot;picPhoneNumber\&quot;: \&quot;505050\&quot;\n}&quot;,
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
      <webElementGuid>71b3ba77-894d-493d-a2d8-09666629c9bd</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a13e6821-1124-4447-8d00-820d1b803d79</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>68268931-03e7-4c4b-aee9-39271c860891</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${rutUrl}${serCen}</restUrl>
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
