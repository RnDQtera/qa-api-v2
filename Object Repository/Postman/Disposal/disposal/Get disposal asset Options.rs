<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get disposal asset Options</name>
   <tag></tag>
   <elementGuidId>824dad9e-a47e-452f-98ac-60c9f38cd943</elementGuidId>
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
      <webElementGuid>9925099e-73b4-44e2-a0a4-23ca1730c6c9</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bt}</value>
      <webElementGuid>34f983cd-2582-4456-9f8c-8374d6a0baba</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${disUrl}${disReq}${opt}?assetNameOptions=true&amp;assetBrandOptions=true&amp;assetModelOptions=true&amp;asset=</restUrl>
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
      <id>a857340c-7539-4d94-bf96-e2ecef5ca193</id>
      <masked>false</masked>
      <name>disUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.disposalRequest</defaultValue>
      <description></description>
      <id>3b2bc1e5-5ed1-4974-b59a-8e7619112919</id>
      <masked>false</masked>
      <name>disReq</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>898abf80-0832-4738-b08a-3e13e0aa5dfc</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Options</defaultValue>
      <description></description>
      <id>29466ae8-ddb6-438c-b9e8-c2eccfd7f5b7</id>
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
