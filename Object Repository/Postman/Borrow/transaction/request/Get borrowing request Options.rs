<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get borrowing request Options</name>
   <tag></tag>
   <elementGuidId>bec5606d-fef1-4458-aa4b-ad5a0ddc4750</elementGuidId>
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
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>bdd09063-550e-40d8-a6e3-79bb80fde753</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>1a32db11-db36-4f75-963d-a67898feab55</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${borUrl}${trReq}${opt}?groupOptions=true&amp;modelOptions=true&amp;brandOptions=true&amp;nameOptions=true</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BORROWING_BASE_URL</defaultValue>
      <description></description>
      <id>5d758f20-c2fd-484d-8230-7d25ec9c6c3e</id>
      <masked>false</masked>
      <name>borUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transactionRequest</defaultValue>
      <description></description>
      <id>c2b86ea9-3c3e-4fa0-b959-44b9186f8e73</id>
      <masked>false</masked>
      <name>trReq</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>b5faa0ff-08c8-4645-9503-5fee086fc965</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Options</defaultValue>
      <description></description>
      <id>1d1b8c01-793e-4768-a2cd-c8b4b90c12da</id>
      <masked>false</masked>
      <name>opt</name>
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
